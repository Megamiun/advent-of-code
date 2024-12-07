package br.com.gabryel.adventofcode.y2023.d19

import br.com.gabryel.adventofcode.readLines
import br.com.gabryel.adventofcode.takeUntilNextBlankLine

private val pipelineBreaker = """(.*)\{(.*)}""".toRegex()
private val pieceBreaker = """\d+""".toRegex()

fun main() {
    listOf("sample", "input").forEach { file ->
        val iterator = readLines(2023, 19, file, keepBlanks = true).listIterator()

        val pipelines = iterator.takeUntilNextBlankLine(false).associate {
            val (_, name, pipelineDesc) = pipelineBreaker.find(it)!!.groupValues
            name to pipelineDesc.asInstruction()
        }

        val pieces = iterator.takeUntilNextBlankLine(false).map {
            val (x, m, a, s) = pieceBreaker.findAll(it).map { it.value.toInt() }.toList()
            Piece(x, m, a, s)
        }

        println("[Approved Sum   ][$file] ${pieces.getSumOfApprovals(pipelines)}")
        println("[Possible Pieces][$file] ${pipelines.getPossiblePieces()}")
    }
}
