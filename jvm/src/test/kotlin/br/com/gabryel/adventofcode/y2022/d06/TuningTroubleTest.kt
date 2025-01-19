package br.com.gabryel.adventofcode.y2022.d06

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class TuningTroubleTest {
    private val files = listOf("sample", "input")
        .associateWith { readLines(2022, 6, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,7", "input,1300"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { extractPositionAndMarker(files[file]!!, 4) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample,19", "input,3986"])
    fun `Part 2`(file: String, expected: Int) {
        val result = timed("Part 2 - $file") { extractPositionAndMarker(files[file]!!, 14) }

        assertThat(result, equalTo(expected))
    }
}