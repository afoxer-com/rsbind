.class Lcom/afoxer/rustlib/MainActivity$1;
.super Ljava/lang/Object;
.source "MainActivity.java"

# interfaces
.implements Lcom/afoxer/xxx/ffi/UploadProgress;


# annotations
.annotation system Ldalvik/annotation/EnclosingMethod;
    value = Lcom/afoxer/rustlib/MainActivity;->onCreate(Landroid/os/Bundle;)V
.end annotation

.annotation system Ldalvik/annotation/InnerClass;
    accessFlags = 0x0
    name = null
.end annotation


# instance fields
.field final synthetic this$0:Lcom/afoxer/rustlib/MainActivity;


# direct methods
.method constructor <init>(Lcom/afoxer/rustlib/MainActivity;)V
    .registers 2
    .param p1, "this$0"    # Lcom/afoxer/rustlib/MainActivity;

    .prologue
    .line 25
    iput-object p1, p0, Lcom/afoxer/rustlib/MainActivity$1;->this$0:Lcom/afoxer/rustlib/MainActivity;

    invoke-direct {p0}, Ljava/lang/Object;-><init>()V

    return-void
.end method


# virtual methods
.method public onProgress(JJJ)V
    .registers 10
    .param p1, "id"    # J
    .param p3, "process"    # J
    .param p5, "total"    # J

    .prologue
    .line 28
    const-string v0, "MainActivity"

    new-instance v1, Ljava/lang/StringBuilder;

    invoke-direct {v1}, Ljava/lang/StringBuilder;-><init>()V

    const-string v2, "upload process is "

    invoke-virtual {v1, v2}, Ljava/lang/StringBuilder;->append(Ljava/lang/String;)Ljava/lang/StringBuilder;

    move-result-object v1

    invoke-virtual {v1, p3, p4}, Ljava/lang/StringBuilder;->append(J)Ljava/lang/StringBuilder;

    move-result-object v1

    invoke-virtual {v1}, Ljava/lang/StringBuilder;->toString()Ljava/lang/String;

    move-result-object v1

    invoke-static {v0, v1}, Landroid/util/Log;->i(Ljava/lang/String;Ljava/lang/String;)I

    .line 29
    return-void
.end method
