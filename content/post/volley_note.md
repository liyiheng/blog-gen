---
date: 2017-03-01
title: "Volley学习笔记"
draft: false
categories:
  - Android
tags:
  - Android
  - Volley
thumbnailImagePosition: left
---



虽然更倾向于retrofit + okhttp，但目前的项目中还在用Volley，从学习的角度扒一扒[Volley的源码](https://github.com/google/volley)吧

主要是分析Volley的部分实现，不再涉及[具体使用和二次封装](https://github.com/liyiheng/EasyVolley)
 
<!--more-->
 

### Volley.java
`Volley.java`用来创建 `RequestQueue`，有两个用来重载静态方法和一个常量（默认的磁盘缓存目录），核心代码如下：
```java
public static RequestQueue newRequestQueue(Context context, HttpStack stack) {
    File cacheDir = new File(context.getCacheDir(), DEFAULT_CACHE_DIR);
    String userAgent = "volley/0";
    try {
        String packageName = context.getPackageName();
        PackageInfo info = context.getPackageManager().getPackageInfo(packageName, 0);
        userAgent = packageName + "/" + info.versionCode;
    } catch (NameNotFoundException ignored) {
    }
    if (stack == null) {
        if (Build.VERSION.SDK_INT >= 9) {
            stack = new HurlStack();
        } else {
            stack = new HttpClientStack(AndroidHttpClient.newInstance(userAgent));
        }
    }
    Network network = new BasicNetwork(stack);
    RequestQueue queue = new RequestQueue(new DiskBasedCache(cacheDir), network);
    queue.start();
    return queue;
}
```
- 1.创建缓存目录
- 2.生成内容为`包名/版本号`的UA，默认为"volley/0"
- 3.如果未指定stack，在api9以上的设备上使用基于HttpURLConnection的stack，更旧是设备上则是用HttpClient实现的stack。传入的HttpStack为`接口类型`，因此可以使用自己的HttpStack实现（如okhttp）。
- 4.创建BasicNetwork对象和DiskBasedCache对象
- 5.通过4中的对象构造一个RequestQueue，并调用start()方法后将其返回

### RequestQueue.java
RequestQueue有一个缓存dispatcher和一组网络dispatcher，其数量（线程池大小）可指定，默认为4.

stop()方法将所有dispatcher停止<br>
```java
public void stop() {
    if (mCacheDispatcher != null) {
        mCacheDispatcher.quit();
    }
    for (int i = 0; i < mDispatchers.length; i++) {
        if (mDispatchers[i] != null) {
            mDispatchers[i].quit();
        }
    }
}
```
start()方法则是先调用stop方法，再创建每一个dispatcher并调用其start方法（dispatcher继承Thread）
```java
public void start() {
    stop();  // Make sure any currently running dispatchers are stopped.
    // Create the cache dispatcher and start it.
    mCacheDispatcher = new CacheDispatcher(mCacheQueue, mNetworkQueue, mCache, mDelivery);
    mCacheDispatcher.start();
    for (int i = 0; i < mDispatchers.length; i++) {
        NetworkDispatcher networkDispatcher = new NetworkDispatcher(mNetworkQueue, mNetwork,
                mCache, mDelivery);
        mDispatchers[i] = networkDispatcher;
        networkDispatcher.start();
    }
}
```
 add()方法将请求添加至队列。
 如果不使用缓存，则直接添加至网络队列并返回，不再使用缓存队列；
 使用缓存时，则先根据cacheKey判断是否有正在等待的相同请求，如果有就加入该key对应的Queue排队，没有则添加到缓存队列并将key对应的Queue置空
```java
public <T> Request<T> add(Request<T> request) {
    // 将该请求标记为属于当前队列，并加入mCurrentRequests
    request.setRequestQueue(this);
    synchronized (mCurrentRequests) {
        mCurrentRequests.add(request);
    }
    // Process requests in the order they are added.
    request.setSequence(getSequenceNumber());
    request.addMarker("add-to-queue");
    // If the request is uncacheable, skip the cache queue and go straight to the network.
    if (!request.shouldCache()) {
        mNetworkQueue.add(request);
        return request;
    }
    // Insert request into stage if there's already a request with the same cache key in flight.
    synchronized (mWaitingRequests) {
        String cacheKey = request.getCacheKey();
        if (mWaitingRequests.containsKey(cacheKey)) {
            // There is already a request in flight. Queue up.
            Queue<Request<?>> stagedRequests = mWaitingRequests.get(cacheKey);
            if (stagedRequests == null) {
                stagedRequests = new LinkedList<Request<?>>();
            }
            stagedRequests.add(request);
            mWaitingRequests.put(cacheKey, stagedRequests);
            if (VolleyLog.DEBUG) {
                VolleyLog.v("Request for cacheKey=%s is in flight, putting on hold.", cacheKey);
            }
        } else {
            // Insert 'null' queue for this cacheKey, indicating there is now a request in
            // flight.
            mWaitingRequests.put(cacheKey, null);
            mCacheQueue.add(request);
        }
        return request;
    }
}
```

### Dispatcher
请求添加到队列后，由Dispatcher处理。NetworkDispatcher和CacheDispatcher都是Thread的子类，主要逻辑在run()方法

- NetworkDispatcher.java核心代码如下
```java
                    // ...
                    // ...
                    // Take a request from the queue.
                    // 此处mQueue的类型为BlockingQueue<Request<?>>，
                    // take()方法有可能阻塞，类似与Linux下的pipe，
                    // Looper.loop()方法中也有同样的用法(生产者消费者模型)
                    request = mQueue.take();
                    // ...
                    // ...
                    // Perform the network request.
                    NetworkResponse networkResponse = mNetwork.performRequest(request);
                    // ...
                    // ...
                    // Parse the response here on the worker thread.
                    Response<?> response = request.parseNetworkResponse(networkResponse);
                    request.addMarker("network-parse-complete");
                    // Write to cache if applicable.
                    // TODO: Only update cache metadata instead of entire record for 304s.
                    if (request.shouldCache() && response.cacheEntry != null) {
                        mCache.put(request.getCacheKey(), response.cacheEntry);
                        request.addMarker("network-cache-written");
                    }
                    // Post the response back.
                    request.markDelivered();
                    // mDelivery通过executor和handler将结果发出，由Looper线程处理，通常是主线程
                    mDelivery.postResponse(request, response);
```   

- CacheDispatcher.java核心代码如下
与NetworkDispatcher的实现类似，但CacheDispatcher需要对缓存数据进行判断，是否存在、是否过期、是否需要刷新等。根据判断结果决定是否需要将请求加入网络队列交给NetworkDispatcher处理
```java
                // Get a request from the cache triage queue, blocking until
                // at least one is available.
                final Request<?> request = mCacheQueue.take();
                // ...
                // ...
                // Attempt to retrieve this item from cache.
                Cache.Entry entry = mCache.get(request.getCacheKey());
                if (entry == null) {
                    request.addMarker("cache-miss");
                    // Cache miss; send off to the network dispatcher.
                    mNetworkQueue.put(request);
                    continue;
                }
                // If it is completely expired, just send it to the network.
                if (entry.isExpired()) {
                    request.addMarker("cache-hit-expired");
                    request.setCacheEntry(entry);
                    mNetworkQueue.put(request);
                    continue;
                }
                // We have a cache hit; parse its data for delivery back to the request.
                request.addMarker("cache-hit");
                Response<?> response = request.parseNetworkResponse(
                        new NetworkResponse(entry.data, entry.responseHeaders));
                request.addMarker("cache-hit-parsed");
                if (!entry.refreshNeeded()) {
                    // Completely unexpired cache hit. Just deliver the response.
                    mDelivery.postResponse(request, response);
                } else {
                    // Soft-expired cache hit. We can deliver the cached response,
                    // but we need to also send the request to the network for
                    // refreshing.
                    // ...
                    // ...
                    mDelivery.postResponse(request, response, new Runnable() {
                        @Override
                        public void run() {
                            try {
                                mNetworkQueue.put(request);
                            } catch (InterruptedException e) {
                                // Not much we can do about this.
                            }
                        }
                    });
                }
```
### 嗯，还没写完

