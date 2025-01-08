package br.com.gabryel.adventofcode.y2023.d19

import br.com.gabryel.adventofcode.y2023.d19.Instruction.Check
import br.com.gabryel.adventofcode.y2023.d19.Instruction.GoTo

val pipelineBreaker = """(.*)\{(.*)}""".toRegex()
val pieceBreaker = """\d+""".toRegex()

fun getSumOfApprovals(groups: List<List<String>>): Int {
    val pipelines = parseInstructions(groups[0])
    val pieces = parsePieces(groups[1])

    return pieces.filter { piece ->
        var current = pipelines["in"]!!
        while (current !is Instruction.Review) {
            current = when (current) {
                is Check -> current.getNextInstruction(piece)
                is GoTo -> pipelines[current.pipeline]!!
                else -> current
            }
        }

        current.approved
    }.sumOf { it.x + it.m + it.a + it.s }
}


