+++
title = "Trusting ssh keys using a centralized hardware secret"
date = "2021-02-06"
category = "notes"
visible = true

[extra]
mastodon = "https://60228.dev/@leo/105687065669022158"
+++
In the past, I've used a single passwordless ssh key shared between all of my devices. This is obviously very insecure, but means that I don't have to deal with an easily-losable hardware token or synchronizing many public keys between devices. However, some little-known functionality makes it possible for me to approve keys with a hardware secret on my workstation and have them automatically be usable to log in to all of my machines.

## AMD fTPM
This feature is in a prominent location on most Ryzen/Threadripper motherboards but isn't clearly explained what exactly it does. However, the functionality it provides is quite useful. The much-maligned AMD PSP gains a helpful use as a TPM, a hardware device which provides quite a few useful cryptography oriented features. The relevant one here is that tpm2-software (originally developed by Intel)'s tpm2-pkcs11 allows using any TPM as a PKCS#11 token.

Getting this working, however, is somewhat of a challenge. Once the fTPM is enabled in the BIOS, the first step is to make sure the kernel can see it. However, I got a `can't request region for resource` error, despite this apparently having been fixed for the fTPM back in 2019.

I thought this issue may have been Threadripper-specific, and I found a patchset that specifically claimed to fix this issue on Threadripper. Due to various issues, it was rejected in favor of the current patch, but none of those issues appeared to be relevant to my case. I ["ported"](https://raw.githubusercontent.com/leo60228/dotfiles/7ca19c07bbed61fc458b04602f476b4c5345470c/files/tpm-threadripper.patch) the patch to newer kernel versions just by reverting the upstreamed patch and then applying the rejected patchset.

Once the TPM is visible, NixOS makes it pretty easy to set up tpm2-pkcs11. All that needs to be done is to add a small snippet to the NixOS configuration that enables tpm2-software as a whole, tpm2-pkcs11 for PKCS#11 emulation, and tabrmd which manages TPM access from userspace:

```nix
{
  security.tpm2 = {
    enable = true;
    pkcs11.enable = true;
    abrmd.enable = true;
  };
}
```

Now that tpm2-pkcs11 is installed, all that needs to be done is to create an emulated PKCS#11 token and generate a key.

```bash
# gives a PID, change the other commands if it isn't 1
tpm2_ptool init

# label, sopin, and userpin can be anything but need to match in future commands
tpm2_ptool addtoken --pid=1 --label=ftpmtoken1 --sopin=mysopin --userpin=myuserpin

# several algorithms are available, see `tpm_ptool --help`
tpm2_ptool addkey --algorithm=ecc256 --label=ftpmtoken1 --sopin=mysopin --userpin=myuserpin
```

## ssh certificates
ssh has a little-known feature where instead of manually distributing ssh public keys to specific users, you can instead have a single CA key that's trusted on each machine. Then, in addition to the private and public keys, you can provide a public certificate created by signing the public key with the CA private key. This signing step is where you specify the users the key can be used for.

In addition, ssh can directly utilize a PKCS#11 key. This is little known (possibly due to strange interactions with `ssh-agent`?) but works perfectly here. The first step is to extract the CA public key:

```bash
ssh-keygen -D /run/current-system/sw/lib/libtpm2_pkcs11.so | tee ssh-ca.pub
```

Next, `ssh-ca.pub` needs to be trusted on all the systems you'd like to log in to. For systems where you can't do this, such as hosted Git providers, you can of course just use `authorized_keys` like normal. NixOS doesn't have an option for this, but it can easily be added to `sshd_config`:

```nix
{
  services.openssh.extraConfig = ''
  TrustedUserCAKeys ${../files/ssh-ca.pub}
  ''; 
}
```

Now, you need to create the signatures. Copy the public keys you'd like to sign over to the system you set up the fTPM on. There are quite a few options available here, with the full list being available in ssh-keygen's manual page. For a simple example with a signature named `desktop` with a serial number of 1 authorized to log in as `leo60228`:

```bash
ssh-keygen -s ssh-ca.pub -I desktop -D /run/current-system/sw/lib/libtpm2_pkcs11.so -n leo60228 -z 1 id_ecdsa.pub
```

This produces an `id_ecdsa-cert.pub` next to `id_ecdsa.pub`. Copy it back over next to `id_ecdsa` on the machine you're logging in with.

At this point, you're done! There's quite a bit more functionality available with both the fTPM and ssh certificates, but this was good enough for what I needed. One common tweak that makes revocations much easier but adds quite a bit of complexity is to set up an automated way to generate certificates with low expiry times, so that you can easily revoke certificates just by refusing to sign them. I didn't find this necessary, however.

### Credits
[The tpm2-pkcs11 docs on SSH](https://github.com/tpm2-software/tpm2-pkcs11/blob/master/docs/SSH.md), [this blog post on using tpm2-pkcs11 with SSH](https://incenp.org/notes/2020/tpm-based-ssh-key.html), and [this blog post from Facebook on ssh certificates](https://engineering.fb.com/2016/09/12/security/scalable-and-secure-access-with-ssh/) were all very helpful in figuring this out.

<a href="https://brid.gy/publish/mastodon"></a>