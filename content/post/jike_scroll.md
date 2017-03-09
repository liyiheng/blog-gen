#### 实现即刻列表中视频封面滚动效果的尝试


#### 粗略实现

重写RecyclerView的onScrolled方法。在RecyclerView滚动时，获取当前屏幕展示的条目，根据需要对其进行操作<br>以ImageView为例，通过scrollTo或scrollBy方法使图片内容发生偏移
```

public class MainActivity extends AppCompatActivity {


    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        MyRecyclerView myRecyclerView = new MyRecyclerView(this);
        ((FrameLayout) findViewById(R.id.activity_main)).addView(myRecyclerView);
        myRecyclerView.setLayoutManager(new LinearLayoutManager(this));
        myRecyclerView.setAdapter(new SampleAdapter());
    }

    class SampleAdapter extends RecyclerView.Adapter<VHolder> {
        @Override
        public VHolder onCreateViewHolder(ViewGroup parent, int viewType) {
            View view = getLayoutInflater().inflate(R.layout.item_qwe, parent, false);
            return new VHolder(view);
        }

        @Override
        public void onBindViewHolder(VHolder holder, int position) {}

        @Override
        public int getItemCount() {return 30;}
    }

    class VHolder extends RecyclerView.ViewHolder {
        ImageView iv;

        public VHolder(View itemView) {
            super(itemView);
            iv = (ImageView) itemView.findViewById(R.id.item_image);
        }

        public void subScroll(int y) {
            int distance = (int) ((y + 5) * 0.1);
            if (-80 > distance || distance > 80) {
                return;
            }
            if (iv != null) {
                iv.scrollBy(0, -distance);
            }
        }

        public void reset() {
            if (iv != null) {
                iv.scrollTo(0, 0);
            }
        }
    }

    class MyRecyclerView extends RecyclerView {

        public MyRecyclerView(Context context) {
            super(context);
        }

        @Override
        public void onChildDetachedFromWindow(View child) {
            super.onChildDetachedFromWindow(child);
            VHolder holder = (VHolder) getChildViewHolder(child);
            holder.reset();
        }

        @Override
        public void onScrolled(int dx, int dy) {
            super.onScrolled(dx, dy);
            LinearLayoutManager layoutManager = ((LinearLayoutManager) getLayoutManager());
            int firstIndex = layoutManager.findFirstVisibleItemPosition();
            int lastIndex = layoutManager.findLastVisibleItemPosition();
            for (int i = 0; i < lastIndex - firstIndex + 1; i++) {
                View childAt = layoutManager.getChildAt(i);
                VHolder holder = (VHolder) getChildViewHolder(childAt);
                holder.subScroll(dy);
            }
        }
    }
}
```
#### 稍作改进

##### "此处不要写死，日后必改"
* 定义一个SubScroller接口。含有scroll,onDetach两个方法，分别在RecyclerView的onScrolled和onChildDetachedFromWindow方法中调用。具体代码如下：
````
    public interface SubScroller {
        void scroll(int x, int y);

        void reset();
    }

    class MyRecyclerView extends RecyclerView {


        public MyRecyclerView(Context context) {
            super(context);
        }

        @Override
        public void onChildDetachedFromWindow(View child) {
            super.onChildDetachedFromWindow(child);
            ViewHolder holder = getChildViewHolder(child);
            if (holder instanceof SubScroller) {
                ((SubScroller) holder).reset();
            }
        }

        @Override
        public void onScrolled(int dx, int dy) {
            super.onScrolled(dx, dy);
            LinearLayoutManager layoutManager = ((LinearLayoutManager) getLayoutManager());
            int firstIndex = layoutManager.findFirstVisibleItemPosition();
            int lastIndex = layoutManager.findLastVisibleItemPosition();
            for (int i = 0; i < lastIndex - firstIndex + 1; i++) {
                View childAt = layoutManager.getChildAt(i);
                ViewHolder holder = getChildViewHolder(childAt);
                if (holder instanceof SubScroller) {
                    ((SubScroller) holder).scroll(dx, dy);
                }
            }
        }
    }
````
* 使用时，只需要使ViewHolder实现SubScroller接口，在ViewHolder中处理具体逻辑。举个栗子：
````
 class VHolder extends RecyclerView.ViewHolder implements SubScroller {
        ImageView iv;

        public VHolder(View itemView) {
            super(itemView);
            iv = (ImageView) itemView.findViewById(R.id.item_image);
        }

        @Override
        public void scroll(int x, int y) {
            if (iv != null) {
                int distance = (int) ((y + 5) * 0.1);
                if (-80 > distance || distance > 80) {
                    return;
                }
                iv.scrollBy(0, -distance);
            }
        }

        @Override
        public void reset() {
            if (iv != null) {
                iv.scrollTo(0, 0);
            }
        }
    }
````
