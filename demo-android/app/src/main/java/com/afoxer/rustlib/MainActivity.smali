.class public Lcom/afoxer/rustlib/MainActivity;
.super Landroid/app/Activity;
.source "MainActivity.java"


# static fields
.field private static final TAG:Ljava/lang/String; = "MainActivity"


# direct methods
.method public constructor <init>()V
    .registers 1

    .prologue
    .line 12
    invoke-direct {p0}, Landroid/app/Activity;-><init>()V

    return-void
.end method


# virtual methods
.method protected onCreate(Landroid/os/Bundle;)V
    .registers 9
    .param p1, "savedInstanceState"    # Landroid/os/Bundle;

    .prologue
    .line 17
    invoke-super {p0, p1}, Landroid/app/Activity;->onCreate(Landroid/os/Bundle;)V

    .line 18
    const/high16 v4, 0x7f010000

    invoke-virtual {p0, v4}, Lcom/afoxer/rustlib/MainActivity;->setContentView(I)V

    .line 19
    invoke-static {}, Lcom/afoxer/xxx/ffi/RustLib;->newServices()Lcom/afoxer/xxx/ffi/Services;

    move-result-object v4

    invoke-interface {v4}, Lcom/afoxer/xxx/ffi/Services;->getLoginService()Lcom/afoxer/xxx/ffi/LoginService;

    move-result-object v1

    .line 20
    .local v1, "loginService":Lcom/afoxer/xxx/ffi/LoginService;
    const-string v4, "sidney.wang"

    const-string v5, "88888888"

    invoke-interface {v1, v4, v5}, Lcom/afoxer/xxx/ffi/LoginService;->login(Ljava/lang/String;Ljava/lang/String;)Lcom/afoxer/xxx/ffi/Future;

    move-result-object v0

    .line 21
    .local v0, "future":Lcom/afoxer/xxx/ffi/Future;
    invoke-interface {v0}, Lcom/afoxer/xxx/ffi/Future;->get()Z

    move-result v2

    .line 22
    .local v2, "result":Z
    const-string v4, "MainActivity"

    new-instance v5, Ljava/lang/StringBuilder;

    invoke-direct {v5}, Ljava/lang/StringBuilder;-><init>()V

    const-string v6, "login result is "

    invoke-virtual {v5, v6}, Ljava/lang/StringBuilder;->append(Ljava/lang/String;)Ljava/lang/StringBuilder;

    move-result-object v5

    invoke-virtual {v5, v2}, Ljava/lang/StringBuilder;->append(Z)Ljava/lang/StringBuilder;

    move-result-object v5

    invoke-virtual {v5}, Ljava/lang/StringBuilder;->toString()Ljava/lang/String;

    move-result-object v5

    invoke-static {v4, v5}, Landroid/util/Log;->i(Ljava/lang/String;Ljava/lang/String;)I

    .line 24
    invoke-static {}, Lcom/afoxer/xxx/ffi/RustLib;->newServices()Lcom/afoxer/xxx/ffi/Services;

    move-result-object v4

    invoke-interface {v4}, Lcom/afoxer/xxx/ffi/Services;->getUploadService()Lcom/afoxer/xxx/ffi/UploadService;

    move-result-object v3

    .line 25
    .local v3, "uploadService":Lcom/afoxer/xxx/ffi/UploadService;
    const-string v4, "to/your/path"

    new-instance v5, Lcom/afoxer/rustlib/MainActivity$1;

    invoke-direct {v5, p0}, Lcom/afoxer/rustlib/MainActivity$1;-><init>(Lcom/afoxer/rustlib/MainActivity;)V

    invoke-interface {v3, v4, v5}, Lcom/afoxer/xxx/ffi/UploadService;->upload(Ljava/lang/String;Lcom/afoxer/xxx/ffi/UploadProgress;)J

    .line 31
    return-void
.end method
