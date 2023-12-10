package br.com.gabryel.adventofcode.y2023.d08

import br.com.gabryel.adventofcode.y2023.readLines
import kotlin.time.measureTimedValue

private val directionRegex = """[0-9A-Z]+""".toRegex()

fun main() {
    listOf("sample", "sample2", "sample3", "input").forEach { file ->
        val lines = readLines(8, file)
        val directions = lines[0]

        val mappings = lines.drop(1).associate {
            val (start, left, right) = directionRegex.findAll(it).map { it.value }.toList()
            start to (left to right)
        }

        if (file != "sample3")
            println("[Steps To Arrive                          ][$file] ${findStepsToArrive(directions, mappings)}")

        // Very, very slow
        val (multiStepResult, time) = measureTimedValue {
            MultiStepMap(directions, mappings).findStepsToArrive()
        }
        println("[Ghostly Steps To Arrive - MultiStep Map  ][$file] $multiStepResult [${time.inWholeMilliseconds} ms]")
    }
}
