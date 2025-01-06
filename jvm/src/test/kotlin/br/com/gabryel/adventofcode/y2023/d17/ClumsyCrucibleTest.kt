package br.com.gabryel.adventofcode.y2023.d17

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class ClumsyCrucibleTest {
    private val files = listOf("sample", "input")
        .associateWith { readLines(2023, 17, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,102", "input,963"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { getMinimumHeatLoss(files[file]!!, 1, 3) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample,94", "input,1178"])
    fun `Part 2`(file: String, expected: Int) {
        val result = timed("Part 2 - $file") { getMinimumHeatLoss(files[file]!!, 4, 10) }

        assertThat(result, equalTo(expected))
    }
}