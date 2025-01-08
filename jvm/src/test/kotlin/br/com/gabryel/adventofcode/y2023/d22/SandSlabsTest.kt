package br.com.gabryel.adventofcode.y2023.d22

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class SandSlabsTest {
    private val files = listOf("sample", "input")
        .associateWith { readLines(2023, 22, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,5", "input,465"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { findNonSingleSupporting(files[file]!!) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample,7", "input,79042"])
    fun `Part 2`(file: String, expected: Int) {
        val result = timed("Part 2 - $file") { findSumOfCausedFalls(files[file]!!) }

        assertThat(result, equalTo(expected))
    }
}