package br.com.gabryel.adventofcode.y2023.d19

import br.com.gabryel.adventofcode.y2023.d19.Operation.GT
import br.com.gabryel.adventofcode.y2023.d19.Operation.LT

sealed class Instruction {
    data class Check(
        val attribute: Char,
        val compare: Operation,
        val value: Int,
        val onSuccess: Instruction,
        val onFailure: Instruction,
    ) : Instruction() {
        fun getNextInstruction(piece: Piece) =
            if (compare(piece.getAttribute(attribute), value)) onSuccess else onFailure
    }

    data class GoTo(val pipeline: String) : Instruction()

    data class Review(val approved: Boolean) : Instruction()
}

enum class Operation(val operation: (Int, Int) -> Boolean) : (Int, Int) -> Boolean by operation {
    GT({ a, b -> a > b }),
    LT({ a, b -> a < b })
}

private val checkFinder = """(.)(.)(\d+):(.+?),(.+)""".toRegex()

fun parseInstructions(lines: List<String>) = lines.associate {
    val (_, name, pipelineDesc) = pipelineBreaker.find(it)!!.groupValues
    name to pipelineDesc.asInstruction()
}

private fun String.asInstruction(): Instruction {
    val checkValues = checkFinder.find(this)?.groupValues?.drop(1)

    checkValues?.let { (attribute, operation, value, onSuccess, onFailure) ->
        val operator = if (operation == "<") LT else GT
        val attr = attribute.first()
        return Instruction.Check(attr, operator, value.toInt(), onSuccess.asInstruction(), onFailure.asInstruction())
    }

    return if (length == 1) Instruction.Review(this == "A")
    else Instruction.GoTo(this)
}
