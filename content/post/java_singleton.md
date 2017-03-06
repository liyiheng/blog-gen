---
date: 2017-03-04
title: "Java学习笔记--单例模式"
draft: false
categories:
  - Java
tags:
  - Java
  - 设计模式
  - design pattern
thumbnailImagePosition: left
---


整理常见的几种单例实现
 
<!--more-->


##### 单例多有静态引用，Android中涉及到Activity、Fragment、Service、View等含有Context的对象须注意防止内存泄露

- 1.最简单的一种，饿汉模式

        class Singleton{
            private static Singleton sInstance = new Singleton();
            private Singleton() {}
        
            public static synchronized Singleton getsInstance() {
                return sInstance;
            }
        }
    
- 2.懒汉模式。第一次使用时实例化，因此需要加锁
    
        class Singleton {
            private static Singleton sInstance;
        
            private Singleton() {}
            
            // 整个方法加上synchronized关键字
            public static synchronized Singleton getsInstance() {
                if (sInstance == null) {
                    sInstance = new Singleton();
                }
                return sInstance;
            }
        }
        
     由于每次调用方法都要同步，开销较大，因此通常使用两次判断加锁的方式实现
        
        class Singleton {
            private static Singleton sInstance;
        
            private Singleton() {}
            // 只在没有实例时加锁以提高性能
            public static Singleton getInstance() {
                if (sInstance == null) {
                    synchronized (Singleton.class) {
                        if (sInstance == null) {
                            sInstance = new Singleton();
                        }
                    }
                }
                return sInstance;
            }
        }
        
- 3.枚举方式，从[何老师](https://github.com/hehonghui)《Android源码设计模式解析与实战》一书中了解到这种方式。枚举单例不仅实现简单，实例的创建也是线程安全的，而且不同于其他几种方式，即使通过序列化和反序列化也不能创建多个实例

        enum Singleton{
            instance,
            public void greeting(){
                System.out.println("hello");
            }
        }
    
- 4.通过InstanceHolder持有实例。这种方式只有第一次调用getInstance()方法时才会创建实例，并且是线程安全的

        class Singleton {
            private Singleton() {}
            public static Singleton getInstance() {
                return Holder.sInstance;
            }
            private static class Holder{
                private static final Singleton sInstance;
            }
        }
        
- 5.使用容器对多个单例统一管理。也是在何老师书中学到，Android源码中对一些系统级服务的管理便是采用这种方式

        class Singletons {
            private static Map<String, Object> sContainer = new HashMap<>();
        
            private Singletons() {}
        
            public static void register(String key, Object instance) {
                if (!sContainer.containsKey(key)) {
                    sContainer.put(key, instance);
                }
            }
        
            public static Object getInstance(String key) {
                return sContainer.get(key);
            }
        }
