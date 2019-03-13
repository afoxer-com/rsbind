package com.afoxer.javagen

import com.google.gson.annotations.SerializedName
import com.squareup.javapoet.ArrayTypeName
import com.squareup.javapoet.ClassName
import com.squareup.javapoet.TypeName

enum class AstType() {
    @SerializedName("Void")
    VOID,
    @SerializedName("Int")
    INT,
    @SerializedName("Int")
    BYTE,
    @SerializedName("Long")
    LONG,
    @SerializedName("Float")
    FLOAT,
    @SerializedName("Double")
    DOUBLE,
    @SerializedName("String")
    STRING,
    @SerializedName("Boolean")
    BOOLEAN,
    @SerializedName("Vec")
    VEC,
    @SerializedName("Callback")
    CALLBACK,
    @SerializedName("Struct")
    STRUCT;
}

fun mapType(str: String): AstType {
    return when (str) {
        "Void" -> AstType.VOID
        "Int" -> AstType.INT
        "Byte" -> AstType.BYTE
        "Long" -> AstType.LONG
        "Float" -> AstType.FLOAT
        "Double" -> AstType.DOUBLE
        "String" -> AstType.STRING
        "Boolean" -> AstType.BOOLEAN
        "Vec" -> AstType.VEC
        "Callback" -> AstType.CALLBACK
        "Struct" -> AstType.STRUCT
        else -> AstType.VOID
    }
}

fun mapType(type: AstType, subType: AstType, transfer: Boolean = false): TypeName {
    return when (type) {
        AstType.BOOLEAN -> if (transfer) TypeName.INT else TypeName.BOOLEAN
        AstType.BYTE -> TypeName.BYTE
        AstType.INT -> TypeName.INT
        AstType.LONG -> TypeName.LONG
        AstType.FLOAT -> TypeName.FLOAT
        AstType.DOUBLE -> TypeName.DOUBLE
        AstType.STRING -> ClassName.get("java.lang", "String")
        AstType.VOID -> TypeName.VOID
        AstType.VEC -> ArrayTypeName.of(mapType(subType, AstType.VOID).box())
        else -> TypeName.VOID
    }
}

data class ArgDesc(val name: String,
                   val ty: AstType,
                   val sub_ty: AstType,
                   val origin_ty: String)

data class MethodDesc(val name: String,
                      val return_type: AstType,
                      val return_sub_type: AstType,
                      val origin_return_ty: String,
                      val args: Array<ArgDesc>)

data class TraitDesc(val name: String,
                     val mod_name: String,
                     val is_callback: Boolean,
                     val methods: Array<MethodDesc>)

data class StructDesc(val name: String,
                      val mod_name: String,
                      val fields: Array<ArgDesc>)

data class AstResult(val traits: Array<TraitDesc>,
                     val structs: Array<StructDesc>)