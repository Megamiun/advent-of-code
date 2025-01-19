package br.com.gabryel.adventofcode.y2022.d02

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class RockPaperScissorsTest {
    private val files = listOf("sample", "input")
        .associateWith { readLines(2022, 2, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,15", "input,12276"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { getRPSHandStancePoints(files[file]!!) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample,12", "input,9975"])
    fun `Part 2`(file: String, expected: Int) {
        val result = timed("Part 2 - $file") { getRPSWinStancePoints(files[file]!!) }

        assertThat(result, equalTo(expected))
    }
}