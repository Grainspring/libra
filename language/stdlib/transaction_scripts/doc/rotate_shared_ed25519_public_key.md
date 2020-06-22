
<a name="SCRIPT"></a>

# Script `rotate_shared_ed25519_public_key.move`

### Table of Contents

-  [Function `rotate_shared_ed25519_public_key`](#SCRIPT_rotate_shared_ed25519_public_key)



<a name="SCRIPT_rotate_shared_ed25519_public_key"></a>

## Function `rotate_shared_ed25519_public_key`



<pre><code><b>public</b> <b>fun</b> <a href="#SCRIPT_rotate_shared_ed25519_public_key">rotate_shared_ed25519_public_key</a>(account: &signer, public_key: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="#SCRIPT_rotate_shared_ed25519_public_key">rotate_shared_ed25519_public_key</a>(account: &signer, public_key: vector&lt;u8&gt;) {
    <a href="../../modules/doc/SharedEd25519PublicKey.md#0x1_SharedEd25519PublicKey_rotate_key">SharedEd25519PublicKey::rotate_key</a>(account, public_key)
}
</code></pre>



</details>