<?php

function my_plugin_enqueue_assets() {
    $plugin_url = plugin_dir_url(__FILE__);

    wp_enqueue_script(
        'my-plugin-js',
        $plugin_url . '../build/index.js',
        array(),
        null,
        true
    );

    wp_enqueue_style(
        'my-plugin-css',
        $plugin_url . '../build/style.css',
        array(),
        null
    );
}

add_action('wp_enqueue_scripts', 'my_plugin_enqueue_assets');
