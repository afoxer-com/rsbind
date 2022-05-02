.class public Lcom/afoxer/rustlib/CustomView;
.super Landroid/view/View;
.source "CustomView.java"


# direct methods
.method public constructor <init>(Landroid/content/Context;)V
    .registers 2
    .param p1, "context"    # Landroid/content/Context;

    .prologue
    .line 8
    invoke-direct {p0, p1}, Landroid/view/View;-><init>(Landroid/content/Context;)V

    .line 9
    return-void
.end method


# virtual methods
.method testView()V
    .registers 2

    .prologue
    .line 12
    const/16 v0, 0x8

    invoke-virtual {p0, v0}, Lcom/afoxer/rustlib/CustomView;->setVisibility(I)V

    .line 13
    return-void
.end method
