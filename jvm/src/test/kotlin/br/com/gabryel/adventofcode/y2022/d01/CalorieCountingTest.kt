package br.com.gabryel.adventofcode.y2022.d01

import br.com.gabryel.adventofcode.util.readGroupsOfLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class CalorieCountingTest {
    private val files = listOf("sample", "input")
        .associateWith { readGroupsOfLines(2022, 1, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,24000", "input,70698"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { findCaloriesForTop(files[file]!!, 1) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample,45000", "input,206643"])
    fun `Part 2`(file: String, expected: Int) {
        val result = timed("Part 2 - $file") { findCaloriesForTop(files[file]!!, 3) }

        assertThat(result, equalTo(expected))
    }
}
