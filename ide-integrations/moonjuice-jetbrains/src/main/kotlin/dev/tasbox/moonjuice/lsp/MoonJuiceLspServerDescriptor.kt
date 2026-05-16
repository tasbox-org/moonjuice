package dev.tasbox.moonjuice.lsp

import com.intellij.execution.configurations.GeneralCommandLine
import com.intellij.openapi.project.Project
import com.intellij.openapi.vfs.VirtualFile
import com.intellij.platform.lsp.api.ProjectWideLspServerDescriptor
import dev.tasbox.moonjuice.language.MoonJuiceFileType
import dev.tasbox.moonjuice.settings.MoonJuiceSettings

class MoonJuiceLspServerDescriptor(project: Project) : ProjectWideLspServerDescriptor(project, "MoonJuice") {
  companion object {
    fun isSupportedFile(file: VirtualFile): Boolean = file.fileType === MoonJuiceFileType
  }

  override fun isSupportedFile(file: VirtualFile): Boolean = MoonJuiceLspServerDescriptor.isSupportedFile(file)

  override fun createCommandLine(): GeneralCommandLine {
    return GeneralCommandLine().apply {
      withParentEnvironmentType(GeneralCommandLine.ParentEnvironmentType.CONSOLE)
      withWorkDirectory(project.basePath)
      withExePath(MoonJuiceSettings.getInstance().lspPath)
    }
  }
}
