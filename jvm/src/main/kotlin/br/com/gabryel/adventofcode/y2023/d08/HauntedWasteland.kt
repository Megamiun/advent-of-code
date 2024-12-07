package br.com.gabryel.adventofcode.y2023.d08

import br.com.gabryel.adventofcode.readLines
import kotlin.time.measureTimedValue

private val directionRegex = """[0-9A-Z]+""".toRegex()

fun main() {
    listOf("sample", "sample2", "sample3", "input").forEach { file ->
        val lines = readLines(2023, 8, file)
        val directions = lines[0]

        val MAPPINGS = lines.drop(1).associate {
            val (start, left, right) = directionRegex.findAll(it).map { it.value }.toList()
            start to (left to right)
        }

        if (file != "sample3")
            println("[Steps To Arrive                          ][$file] ${findStepsToArrive(directions, MAPPINGS)}")

        // Very, very slow
        val (multiStepResult, time) = measureTimedValue {
            MultiStepMap(directions, MAPPINGS).findStepsToArrive()
        }
        println("[Ghostly Steps To Arrive - MultiStep Map  ][$file] $multiStepResult [${time.inWholeMilliseconds} ms]")
    }
}
