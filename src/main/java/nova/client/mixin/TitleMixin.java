package nova.client.mixin;
import net.minecraft.client.Minecraft;
import net.minecraft.client.gui.GuiGraphicsExtractor;
import net.minecraft.client.gui.screens.TitleScreen;
import nova.client.NovaTheme;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
@Mixin(TitleScreen.class)
public abstract class TitleMixin {
    @Inject(method="extractBackground",at=@At("HEAD"),cancellable=true)
    private void nova$background(GuiGraphicsExtractor g,int x,int y,float delta,CallbackInfo ci){
        NovaTheme.background(g); ci.cancel();
    }
    @Inject(method="extractRenderState",at=@At("TAIL"))
    private void nova$brand(GuiGraphicsExtractor g,int x,int y,float delta,CallbackInfo ci){
        g.text(Minecraft.getInstance().font,"NOVA / 0.1",10,10,0xffd9ccff,true);
        NovaTheme.stripe(g,10,23,70);
    }
}
