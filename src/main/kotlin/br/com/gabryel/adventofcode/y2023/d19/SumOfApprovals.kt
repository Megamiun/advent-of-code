package br.com.gabryel.adventofcode.y2023.d19

import br.com.gabryel.adventofcode.y2023.d19.Instruction.Check
import br.com.gabryel.adventofcode.y2023.d19.Instruction.GoTo

fun List<Piece>.getSumOfApprovals(pipelines: Map<String, Instruction>) =
    filter { piece ->
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
