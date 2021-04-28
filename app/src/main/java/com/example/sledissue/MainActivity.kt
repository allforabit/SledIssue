package com.example.sledissue

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.util.Log

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        System.loadLibrary("rust")

        val filename = "${filesDir.absolutePath}/sled-db"
        val didInit = setupSledStore(filename)

        Log.i(TAG, "Did init $didInit")

        // 1. update sled store as normal
        val updateResult = updateSledStore()
        Log.i(TAG, "Update result $updateResult")

        // 2. Comment out above and uncomment below and restart app
//        val query = querySledStore()
//        Log.i(TAG, "Query result $query")

        // 3. Now comment out both above and uncomment below and restart app
//        val updateWithFlushResult = updateSledStoreWithFlush()
//        Log.i(TAG, "Update result $updateWithFlushResult")

        // 4. Comment out above and uncomment no 2 to requery db
        // Db will have persisted due to the flushing

    }

    // Sled test
    external fun setupSledStore(path: String): String

    external fun updateSledStore(): String

    external fun updateSledStoreWithFlush(): String

    external fun querySledStore(): String

    companion object {
        const val TAG = "RUST"
    }

}