package org.lnpbp.demoapp;

import android.os.AsyncTask;
import android.os.Bundle;
import android.util.Log;
import android.view.Menu;
import android.view.MenuItem;
import android.view.View;

import androidx.appcompat.app.AppCompatActivity;
import androidx.appcompat.widget.Toolbar;

import com.google.android.material.floatingactionbutton.FloatingActionButton;

import org.lnpbp.rgb.Runtime;
import org.lnpbp.rgb.model.OutpointCoins;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashSet;
import java.util.UUID;

public class MainActivity extends AppCompatActivity {

    private static final String TAG = MainActivity.class.getSimpleName();

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        Toolbar toolbar = findViewById(R.id.toolbar);
        setSupportActionBar(toolbar);

        final FloatingActionButton fab = findViewById(R.id.fab);
        fab.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                new Thread(new Runnable() {
                    @Override
                    public void run() {
                        final DemoApp app = (DemoApp) getApplication();
                        final Runtime runtime = app.getRuntime();
                        try {

                            final OutpointCoins allocation = new OutpointCoins(
                                100L,
                                "0313ba7cfcaa66029a1a63918ebc426259f00953016c461663315d1bf6b83ab4:0"
                            );
                            byte assetPrecision = 8;
                            runtime.issue(
                                "DEMO",
                                "Demo token",
                                "Token description", assetPrecision,
                                Arrays.asList(allocation), new HashSet<OutpointCoins>(),
                                null, null
                            );
                            Log.d(TAG, "issue completed");

                            /*
                            final String consignment_file = app.dataDir + "/" + UUID.randomUUID().toString();
                            final String transaction_file = app.dataDir + "/" + UUID.randomUUID().toString();
                            final String invoice = "rgb20:outpoint1mzu8vz3jly3rzzkdpph583yahv9wktljtfcln6pe2le6n7ehqulstu967t?amount=5&asset=rgb:id1yqqqxya60n725eszngdx8yvwh3pxyk0sp9fszmzxze3nzhgm76ur4dqf2f7gy";
                            final String prototypePsbt = "cHNidP8BAFICAAAAAZ38ZijCbFiZ/hvT3DOGZb/VXXraEPYiCXPfLTht7BJ2AQAAAAD/////AfA9zR0AAAAAFgAUezoAv9wU0neVwrdJAdCdpu8TNXkAAAAATwEENYfPAto/0AiAAAAAlwSLGtBEWx7IJ1UXcnyHtOTrwYogP/oPlMAVZr046QADUbdDiH7h1A3DKmBDck8tZFmztaTXPa7I+64EcvO8Q+IM2QxqT64AAIAAAACATwEENYfPAto/0AiAAAABuQRSQnE5zXjCz/JES+NTzVhgXj5RMoXlKLQH+uP2FzUD0wpel8itvFV9rCrZp+OcFyLrrGnmaLbyZnzB1nHIPKsM2QxqT64AAIABAACAAAEBKwBlzR0AAAAAIgAgLFSGEmxJeAeagU4TcV1l82RZ5NbMre0mbQUIZFuvpjIBBUdSIQKdoSzbWyNWkrkVNq/v5ckcOrlHPY5DtTODarRWKZyIcSEDNys0I07Xz5wf6l0F1EFVeSe+lUKxYusC4ass6AIkwAtSriIGAp2hLNtbI1aSuRU2r+/lyRw6uUc9jkO1M4NqtFYpnIhxENkMak+uAACAAAAAgAAAAAAiBgM3KzQjTtfPnB/qXQXUQVV5J76VQrFi6wLhqyzoAiTACxDZDGpPrgAAgAEAAIAAAAAAACICA57/H1R6HV+S36K6evaslxpL0DukpzSwMVaiVritOh75EO3kXMUAAACAAAAAgAEAAIAA";
                            final String[] inputs = new String[]{"0313ba7cfcaa66029a1a63918ebc426259f00953016c461663315d1bf6b83ab4:0"};
                            runtime.transfer(
                                Arrays.asList(inputs), Arrays.asList(allocation), invoice,
                                prototypePsbt, consignment_file, transaction_file);
                            Log.d(TAG, "transfer completed");
                            */

                            String assets = runtime.outpointAssets("5aa2d0a8098371ee12b4b59f43ffe6a2de637341258af65936a5baa01da49e9b:0");
                            Log.i(TAG, "assets: " + assets);
                        } catch (Exception e) {
                            Log.e(TAG, e.getMessage());
                        }
                    }
                }).start();

            }
        });
    }

    @Override
    public boolean onCreateOptionsMenu(Menu menu) {
        // Inflate the menu; this adds items to the action bar if it is present.
        getMenuInflater().inflate(R.menu.menu_main, menu);
        return true;
    }

    @Override
    public boolean onOptionsItemSelected(MenuItem item) {
        // Handle action bar item clicks here. The action bar will
        // automatically handle clicks on the Home/Up button, so long
        // as you specify a parent activity in AndroidManifest.xml.
        int id = item.getItemId();

        //noinspection SimplifiableIfStatement
        if (id == R.id.action_settings) {
            return true;
        }

        return super.onOptionsItemSelected(item);
    }
}
