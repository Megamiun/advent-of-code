package br.com.gabryel.adventofcode.y2023.d01

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class TrebuchetTest {
    private val files = listOf("sample", "sample-2", "input")
        .associateWith { readLines(2023, 1, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,142", "input,54697"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { sumTrebuchetCalibrations(files[file]!!) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample-2,281", "input,54885"])
    fun `Part 2`(file: String, expected: Int) {
        val result = timed("Part 2 - $file") { sumTrebuchetCalibrationsWithWrittenNumbers(files[file]!!) }

        assertThat(result, equalTo(expected))
    }
}