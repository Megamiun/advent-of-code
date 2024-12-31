package br.com.gabryel.adventofcode.y2023.d03

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class GearRatiosTest {
    private val files = listOf("sample", "input")
        .associateWith { readLines(2023, 3, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,4361", "input,550934"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { getSumOfGearPartNumbers(files[file]!!) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample,467835", "input,81997870"])
    fun `Part 2`(file: String, expected: Int) {
        val result = timed("Part 2 - $file") { getSumOfGearRatios(files[file]!!) }

        assertThat(result, equalTo(expected))
    }
}