package nova.nativeclient;
import java.lang.reflect.Method;
public final class Theme {
 private static Method width,height,fill,gradient;
 private static boolean failed;
 private static void init(Object g)throws Exception{
  if(width!=null)return;
  Class<?> c=g.getClass();
  width=c.getMethod("guiWidth");height=c.getMethod("guiHeight");
  fill=c.getMethod("fill",int.class,int.class,int.class,int.class,int.class);
  gradient=c.getMethod("fillGradient",int.class,int.class,int.class,int.class,int.class,int.class);
 }
 public static void background(Object g){
  if(failed)return;
  try{init(g);int w=(Integer)width.invoke(g),h=(Integer)height.invoke(g);
   gradient.invoke(g,0,0,w,h,0xff121326,0xff2a1940);
   double t=System.nanoTime()/1e9;
   for(int i=0;i<16;i++){int x=(int)((i*.6180339%1)*w),y=(int)(((i*.381966+t*.014)%1)*h);fill.invoke(g,x,y,x+2,y+2,0x90b59cff);}
  }catch(Exception e){failed=true;System.err.println("[Nova] UI unavailable: "+e.getClass().getSimpleName());}
 }
 public static void loading(Object g){
  if(failed)return;
  try{init(g);int w=(Integer)width.invoke(g),h=(Integer)height.invoke(g);
   int x=Math.max(0,w/2-60),y=Math.max(0,h-20),a=Math.min(120,w);
   fill.invoke(g,x,y,x+a,y+3,0xff3b2d57);
   int offset=(int)((a*.7)*(1+Math.sin(System.nanoTime()/5e8))/2);
   fill.invoke(g,x+offset,y,x+Math.min(a,offset+Math.max(1,a/4)),y+3,0xffb59cff);
  }catch(Exception e){failed=true;System.err.println("[Nova] Loading UI unavailable");}
 }
}
