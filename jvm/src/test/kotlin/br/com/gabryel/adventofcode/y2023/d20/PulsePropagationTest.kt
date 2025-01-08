package br.com.gabryel.adventofcode.y2023.d20

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class PulsePropagationTest {
    private val files = listOf("sample", "sample-2", "input")
        .associateWith { readLines(2023, 20, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,32000000", "sample-2,11687500", "input,873301506"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { getSignalMultiplication(files[file]!!) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["input,241823802412393"])
    fun `Part 2`(file: String, expected: Long) {
        val result = timed("Part 2 - $file") { getFirstCallToRx(files[file]!!) }

        assertThat(result, equalTo(expected))
    }
}