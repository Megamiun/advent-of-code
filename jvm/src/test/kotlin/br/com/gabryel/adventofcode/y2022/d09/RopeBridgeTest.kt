package br.com.gabryel.adventofcode.y2022.d09

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class RopeBridgeTest {
    private val files = listOf("sample", "sample-2", "input")
        .associateWith { readLines(2022, 9, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,13", "input,6367"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { getMotionsForKnots(files[file]!!, 2) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample-2,36", "input,2536"])
    fun `Part 2`(file: String, expected: Int) {
        val result = timed("Part 2 - $file") { getMotionsForKnots(files[file]!!, 10) }

        assertThat(result, equalTo(expected))
    }
}