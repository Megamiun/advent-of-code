package br.com.gabryel.adventofcode.y2022.d08

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class TreetopTreeHouseTest {
    private val files = listOf("sample", "input")
        .associateWith { readLines(2022, 8, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,21", "input,1816"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { getVisibilityFromOutside(files[file]!!) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample,8", "input,383520"])
    fun `Part 2`(file: String, expected: Int) {
        val result = timed("Part 2 - $file") { getVisibilityFromInside(files[file]!!) }

        assertThat(result, equalTo(expected))
    }
}