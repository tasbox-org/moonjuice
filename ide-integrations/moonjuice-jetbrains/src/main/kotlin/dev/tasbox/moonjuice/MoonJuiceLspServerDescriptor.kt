package dev.tasbox.moonjuice

import com.intellij.execution.ExecutionException
import com.intellij.execution.configurations.GeneralCommandLine
import com.intellij.openapi.application.PluginPathManager
import com.intellij.openapi.project.Project
import com.intellij.openapi.vfs.VirtualFile
import com.intellij.platform.lsp.api.ProjectWideLspServerDescriptor

class MoonJuiceLspServerDescriptor(project: Project) : ProjectWideLspServerDescriptor(project, "MoonJuice") {
  companion object {
    fun isSupportedFile(file: VirtualFile): Boolean = file.fileType === MoonJuiceFileType
  }

  override fun isSupportedFile(file: VirtualFile): Boolean = MoonJuiceLspServerDescriptor.isSupportedFile(file)

  override fun createCommandLine(): GeneralCommandLine {
    // TODO: Handle platform-specific binaries
    val lsp = PluginPathManager.getPluginResource(javaClass, "lsp/moonjuice-lsp")
    if (lsp == null || !lsp.exists()) {
      throw ExecutionException("Missing language server executable for target platform")
    }

    return GeneralCommandLine().apply {
      withParentEnvironmentType(GeneralCommandLine.ParentEnvironmentType.CONSOLE)
      withWorkDirectory(project.basePath)
      withExePath(lsp.path)
    }
  }
}
