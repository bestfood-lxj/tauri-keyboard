package com.root.tauri_android_keyboard

import android.os.Bundle
import androidx.activity.enableEdgeToEdge
import android.view.inputmethod.InputMethodManager
import android.view.WindowManager
import android.content.Context

class MainActivity : TauriActivity() {
  private var isKeyboardForced = false
  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
    // Very important for Tauri + WebView
    window.setSoftInputMode(
        WindowManager.LayoutParams.SOFT_INPUT_STATE_ALWAYS_VISIBLE or
        WindowManager.LayoutParams.SOFT_INPUT_ADJUST_RESIZE
    )
  }
  override fun onWindowFocusChanged(hasFocus: Boolean) {
      super.onWindowFocusChanged(hasFocus)
      if (hasFocus && !isKeyboardForced) {
          forceShowKeyboardOnce()
      }
  }

  private fun forceShowKeyboardOnce() {
      if (isKeyboardForced) return

      val imm = getSystemService(Context.INPUT_METHOD_SERVICE) as InputMethodManager

      // This combination works perfectly with Tauri/WebView without blinking
      imm.toggleSoftInput(InputMethodManager.SHOW_FORCED, 0)

      // Mark as forced so we don't call it again and again
      isKeyboardForced = true

      // Safety: if for any reason it gets hidden, re-show once after 1 second
      window.decorView.postDelayed({
          if (isFinishing || isDestroyed) return@postDelayed
          imm.toggleSoftInput(InputMethodManager.SHOW_FORCED, 0)
      }, 1000)
  }
  override fun onResume() {
      super.onResume()
      // Start forcing keyboard when app becomes visible
      forceShowKeyboardOnce()
  }

    override fun onPause() {
        super.onPause()
        hideKeyboard()
        isKeyboardForced = false
    }

    override fun onStop() {
        super.onStop()
        hideKeyboard() // Extra safety
    }

    private fun hideKeyboard() {
        val imm = getSystemService(Context.INPUT_METHOD_SERVICE) as InputMethodManager
        imm.hideSoftInputFromWindow(window.decorView.windowToken, 0)
    }
}
