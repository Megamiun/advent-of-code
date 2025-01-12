package br.com.gabryel.adventofcode.y2024.d12

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class GardenGroupsTest {
    private val files = listOf("sample", "input")
        .associateWith { readLines(2024, 12, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,1930", "input,1450816"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { findGardenPricePerPerimeter(files[file]!!) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample,1206", "input,865662"])
    fun `Part 2`(file: String, expected: Int) {
        val result = timed("Part 2 - $file") { findGardenPricePerSides(files[file]!!) }

        assertThat(result, equalTo(expected))
    }
}
