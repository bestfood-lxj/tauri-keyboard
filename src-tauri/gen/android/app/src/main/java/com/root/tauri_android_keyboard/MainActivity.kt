package com.root.tauri_android_keyboard

import android.os.Bundle
import androidx.activity.enableEdgeToEdge

class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
    showSoftKeyboard()
  }
  fun showSoftKeyboard(view: View) {
   if (view.requestFocus()) {
       val imm = getSystemService(InputMethodManager::class.java)
       imm.showSoftInput(this, InputMethodManager.SHOW_FORCED)
   }
  }
}
