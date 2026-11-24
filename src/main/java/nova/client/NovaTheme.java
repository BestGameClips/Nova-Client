package nova.client;
import net.minecraft.client.gui.GuiGraphicsExtractor;
public final class NovaTheme {
    private NovaTheme() {}
    public static void background(GuiGraphicsExtractor g) {
        int w=g.guiWidth(), h=g.guiHeight();
        g.fillGradient(0,0,w,h,0xff10152b,0xff28183f);
        double t=System.nanoTime()/1e9;
        for(int i=0;i<48;i++) {
            int x=(int)((i*0.61803398875%1)*w);
            int y=(int)(((i*0.381966+t*(0.012+i%5*0.003))%1)*h);
            int alpha=(int)(65+35*Math.sin(t*1.7+i));
            int r=1+i%2;
            g.fill(x,y,x+r,y+r,(alpha<<24)|0xb4a0ff);
        }
    }
    public static void stripe(GuiGraphicsExtractor g,int x,int y,int w) {
        if(w<2)return;
        g.fill(x,y,x+w,y+2,0xff44365d);
        int length=Math.max(1,w/4);
        double wave=(1+Math.sin(System.nanoTime()/5e8))/2;
        int offset=(int)(wave*(w-length));
        g.fill(x+offset,y,x+offset+length,y+2,0xffb59cff);
    }
}
