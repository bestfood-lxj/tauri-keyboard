package com.root.tauri_android_keyboard

import android.os.Bundle
import androidx.activity.enableEdgeToEdge
import android.view.inputmethod.InputMethodManager
import android.view.WindowManager
import android.content.Context

class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
     window.setSoftInputMode(WindowManager.LayoutParams.SOFT_INPUT_STATE_ALWAYS_VISIBLE)
  }
  private val forceShowKeyboardRunnable = object : Runnable {
    override fun run() {
        val imm = getSystemService(Context.INPUT_METHOD_SERVICE) as InputMethodManager
        imm.toggleSoftInput(InputMethodManager.SHOW_FORCED, 0)
        // Keep forcing every 600ms – this survives everything
        window.decorView.postDelayed(this, 600)
    }
  }
    override fun onResume() {
        super.onResume()
        // Start forcing keyboard when app becomes visible
        window.decorView.post(forceShowKeyboardRunnable)
    }

    override fun onPause() {
        super.onPause()
        // Stop forcing + hide keyboard immediately when leaving
        window.decorView.removeCallbacks(forceShowKeyboardRunnable)
        hideKeyboard()
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
