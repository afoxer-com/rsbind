package com.bytedance.ee.bear.javagen

import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.content
import java.io.File


class AstParser(val astPath: String) {
    fun parseAst(): AstResult {
        val traitList = mutableListOf<TraitDesc>()
        val structList = mutableListOf<StructDesc>()
        val dir = File(this.astPath)
        val files = dir.listFiles();
        for (file in files) {
            val content = loadAstJson(file.absolutePath)

            val jsonElement = Json.plain.parseJson(content)
            val type = jsonElement.jsonObject["ty"].primitive.content
            if (type == "trait") {
                traitList.add(parseTrait(jsonElement))
            } else if (type == "struct") {
                structList.add(parseStruct(jsonElement))
            }
        }

        val astResult = AstResult(traitList.toTypedArray(), structList.toTypedArray())

        println("parsed json ast => $astResult")
        return astResult
    }

    private fun loadAstJson(filePath: String): String {
        println("begin load ast json for ${filePath}")
        val file = File(filePath);
        return file.readText();
    }

    private fun parseStruct(jsonElement: JsonElement): StructDesc {
        val structName = jsonElement.jsonObject["name"].primitive.content
        val modName = jsonElement.jsonObject["mod_name"].primitive.content
        val fields = mutableListOf<ArgDesc>()
        val fieldsArray = jsonElement.jsonObject["fields"].jsonArray
        for (fieldEach in fieldsArray) {
            val fieldElement = fieldEach.jsonObject
            val fieldName = fieldElement["name"].content
            val fieldOriginTy = fieldElement["origin_ty"].content
            val fieldTypeJson = fieldElement["ty"];
            val (fieldType, fieldSubType) = parseType(fieldTypeJson)
            val field = ArgDesc(fieldName, fieldType, fieldSubType, fieldOriginTy)
            fields.add(field)
        }

        return StructDesc(structName, modName, fields.toTypedArray())
    }

    private fun parseTrait(jsonElement: JsonElement): TraitDesc {
        val traitName = jsonElement.jsonObject["name"].primitive.content
        val modName = jsonElement.jsonObject["mod_name"].primitive.content
        val isCallback = jsonElement.jsonObject["is_callback"].primitive.boolean

        val methodsArray = jsonElement.jsonObject["methods"].jsonArray
        val methodList = mutableListOf<MethodDesc>()
        var index = 0
        while (index < methodsArray.size) {
            val methodElement = methodsArray[index].jsonObject
            val methodName = methodElement["name"].primitive.content
            val methodRetType = methodElement["return_type"]

            val (returnType, returnSubType) = parseType(methodRetType)
            val originRetType = methodElement["origin_return_ty"].primitive.content;

            val argArray = methodElement["args"].jsonArray

            val argList = mutableListOf<ArgDesc>()
            var i = 0
            while (i < argArray.size) {
                val argElement = argArray[i].jsonObject
                val argName = argElement["name"].content
                val argOriginTy = argElement["origin_ty"].content
                val argItem = argElement["ty"]
                val (argType, argSubType) = parseType(argItem)
                val argDesc = ArgDesc(argName, argType, argSubType, argOriginTy)
                argList.add(argDesc)
                i++
            }

            val methodDesc = MethodDesc(methodName, returnType, returnSubType, originRetType, argList.toTypedArray())
            methodList.add(methodDesc)
            index++
        }

        return TraitDesc(traitName, modName, isCallback, methodList.toTypedArray())
    }

    private fun parseType(tyElement: JsonElement): Pair<AstType, AstType> {
        val argType: AstType
        var argSubType = AstType.VOID
        if (tyElement is JsonObject) {
            val ty = tyElement.jsonObject["Vec"].content
            argType = mapType("Vec")
            argSubType = mapType(ty)
        } else {
            val ty = tyElement.content
            argType = mapType(ty)
        }

        return Pair(argType, argSubType)
    }
}