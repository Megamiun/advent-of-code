package br.com.gabryel.adventofcode.y2022

import br.com.gabryel.adventofcode.util.getLinesFromSystemIn

fun main() {
    val positions = sequence {
        var current = 1
        yield(current)
        getInstructions().forEach {
            yield(current)

            if (it == "noop") return@forEach

            val (_, value) = it.split(" ")
            current += value.toInt()
            yield(current)
        }
    }.toList()

    val sumOfProducts = positions.mapIndexed { index, value -> (index + 1) * value }
        .filterIndexed { index, _ -> index % 40 == 19 }
        .sum()

    println("Product of chosen frequencies: $sumOfProducts")

    (1..240).zip(positions).map { (cycle, spriteCenter) ->
        val cursorPosition = (cycle - 1) % 40

        if (cursorPosition == 0) println()
        if ((cursorPosition - spriteCenter) in (-1..1)) print("#")
        else print(".")
    }
}

private fun getInstructions() = getLinesFromSystemIn()
