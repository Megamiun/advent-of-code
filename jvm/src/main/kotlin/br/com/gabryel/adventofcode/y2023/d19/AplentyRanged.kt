package br.com.gabryel.adventofcode.y2023.d19

import br.com.gabryel.adventofcode.y2023.d19.Instruction.*

private val totalRange = (1..4000).toList()

fun getPossiblePieces(groups: List<List<String>>): Long {
    val parseInstructions = parseInstructions(groups[0])

    return parseInstructions.countApprovable(
        parseInstructions["in"]!!,
        mapOf('x' to totalRange, 'm' to totalRange, 'a' to totalRange, 's' to totalRange)
    )
}

private fun Map<String, Instruction>.countApprovable(instruction: Instruction, possible: Map<Char, List<Int>>): Long =
    when (instruction) {
        is Review -> if (instruction.approved) possible.values.fold(1) { acc, curr -> acc * curr.size } else 0
        is GoTo -> countApprovable(this[instruction.pipeline]!!, possible)
        is Check ->
            countApprovable(instruction.onSuccess, instruction.removeUnmatchedRange(possible, true)) +
                    countApprovable(instruction.onFailure, instruction.removeUnmatchedRange(possible, false))
    }

private fun Check.removeUnmatchedRange(possible: Map<Char, List<Int>>, matches: Boolean) =
    possible.mapValues { (attr, previousPossible) ->
        if (attr != attribute) previousPossible
        else previousPossible.filter { compare(it, value) == matches }
    }
