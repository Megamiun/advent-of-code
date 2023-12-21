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

enum class Operation: (Int, Int) -> Boolean {
    GT { override fun invoke(a: Int, b: Int) = a > b },
    LT { override fun invoke(a: Int, b: Int) = a < b },
}

private val checkFinder = """(.)(.)(\d+):(.+?),(.+)""".toRegex()

fun String.asInstruction(): Instruction {
    val checkValues = checkFinder.find(this)?.groupValues?.drop(1)

    checkValues?.let { (attribute, operation, value, onSuccess, onFailure) ->
        val operator = if (operation == "<") LT else GT
        val attr = attribute.first()
        return Instruction.Check(attr, operator, value.toInt(), onSuccess.asInstruction(), onFailure.asInstruction())
    }

    return if (length == 1) Instruction.Review(this == "A")
    else Instruction.GoTo(this)
}
