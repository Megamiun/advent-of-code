package br.com.gabryel.adventofcode.y2023.d15

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class LensLibraryTest {
    private val files = listOf("sample", "input")
        .associateWith { readLines(2023, 15, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,1320", "input,517551"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { findSumOfLensHashes(files[file]!!) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample,145", "input,286097"])
    fun `Part 2`(file: String, expected: Int) {
        val result = timed("Part 2 - $file") { findFocalPower(files[file]!!) }

        assertThat(result, equalTo(expected))
    }
}