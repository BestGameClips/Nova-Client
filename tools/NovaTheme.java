package nova;
import net.minecraft.client.gui.GuiGraphicsExtractor;
public final class NovaTheme {
 public static boolean background(GuiGraphicsExtractor g){
  int w=g.guiWidth(),h=g.guiHeight();
  g.fillGradient(0,0,w,h,0xff101326,0xff29183d);
  double t=System.nanoTime()/1e9;
  for(int i=0;i<12;i++){int x=(int)((i*.6180339%1)*w),y=(int)(((i*.381966+t*.014)%1)*h);g.fill(x,y,x+2,y+2,0x90b59cff);}
  return true;
 }
 public static void loading(GuiGraphicsExtractor g){
  int w=g.guiWidth(),h=g.guiHeight(),a=Math.min(120,w),x=(w-a)/2,y=Math.max(0,h-20);
  g.fill(x,y,x+a,y+3,0xff3b2d57);
  int offset=(int)((a*.7)*(1+Math.sin(System.nanoTime()/5e8))/2);
  g.fill(x+offset,y,x+Math.min(a,offset+Math.max(1,a/4)),y+3,0xffb59cff);
 }
}
