package dev.tasbox.moonjuice

import com.intellij.openapi.fileTypes.LanguageFileType
import com.intellij.openapi.util.NlsContexts
import com.intellij.openapi.util.NlsSafe
import org.jetbrains.annotations.NonNls
import javax.swing.Icon

object MoonJuiceFileType : LanguageFileType(MoonJuiceLanguage) {
  override fun getName(): @NonNls String = "MoonJuice"

  override fun getDescription(): @NlsContexts.Label String = "MoonJuice script"

  override fun getDefaultExtension(): @NlsSafe String = "mj"

  override fun getIcon(): Icon = MoonJuiceIcons.Language
}
