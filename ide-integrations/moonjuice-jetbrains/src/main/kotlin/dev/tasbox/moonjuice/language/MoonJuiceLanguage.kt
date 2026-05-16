package dev.tasbox.moonjuice.language

import com.intellij.lang.Language
import com.intellij.openapi.util.NlsSafe

object MoonJuiceLanguage : Language("MoonJuice") {
  override fun getDisplayName(): @NlsSafe String = "MoonJuice"
}
