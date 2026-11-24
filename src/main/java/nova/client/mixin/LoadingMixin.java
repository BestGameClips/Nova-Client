package nova.client.mixin;
import net.minecraft.client.Minecraft;
import net.minecraft.client.gui.GuiGraphicsExtractor;
import net.minecraft.client.gui.screens.LoadingOverlay;
import nova.client.NovaTheme;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
@Mixin(LoadingOverlay.class)
public abstract class LoadingMixin {
    @Inject(method="extractRenderState",at=@At("TAIL"))
    private void nova$loading(GuiGraphicsExtractor g,int x,int y,float delta,CallbackInfo ci){
        int w=g.guiWidth();
        g.fillGradient(0,0,w,34,0xff10152b,0xff28183f);
        g.centeredText(Minecraft.getInstance().font,"NOVA CLIENT",w/2,9,0xffe3d9ff);
        NovaTheme.stripe(g,Math.max(0,w/2-55),23,Math.min(w,110));
    }
}
