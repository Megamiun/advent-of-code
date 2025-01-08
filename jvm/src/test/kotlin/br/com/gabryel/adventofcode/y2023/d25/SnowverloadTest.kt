package br.com.gabryel.adventofcode.y2023.d25

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class SnowverloadTest {
    private val files = listOf("sample", "input")
        .associateWith { readLines(2023, 25, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,54", "input,554064"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { getProductsOfSnowverload(files[file]!!) }

        assertThat(result, equalTo(expected))
    }
}