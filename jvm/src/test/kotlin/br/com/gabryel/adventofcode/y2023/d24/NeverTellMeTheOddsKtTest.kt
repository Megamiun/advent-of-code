package br.com.gabryel.adventofcode.y2023.d24

import br.com.gabryel.adventofcode.util.readLines
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource
import kotlin.time.measureTimedValue

class NeverTellMeTheOddsKtTest {

    private val files = listOf("sample", "input")
        .associateWith { readLines(2023, 24, it) }

    @ParameterizedTest(name = "Part 1 - {0} - [{1}; {2}]")
    @CsvSource(
        value = [
            "sample,7,27,2",
            "input,200000000000000,400000000000000,16050"
        ]
    )
    fun `Part 1`(file: String, min: Long, max: Long, expected: Int) {
        val result = timed("Part 1 - $file") { countFutureIntersections(files[file]!!, min, max) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(
        value = [
            "sample,47",
            "input,669042940632377"
        ]
    )
    fun `Part 2`(file: String, expected: Long) {
        val result = timed("Part 2 - $file") { getSumOf(files[file]!!) }

        assertThat(result, equalTo(expected))
    }
}

private fun <T> timed(name: String, exec: () -> T): T {
    println("-------------------")
    println("-------------------")
    println(name)
    println("-------------------")
    println("-------------------")

    val (result, timeTaken) = measureTimedValue(exec)

    println()
    println("[$name] Result: $result")
    println("[$name] Took $timeTaken")

    return result
}
