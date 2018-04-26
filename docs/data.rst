=====================
Visplay Data Protocol
=====================

Overview
========

The VPD defines a method of synchronizing serialized description objects across
networks.

Definitions
-----------

description
    An object containing some arbitrary data as a key-value list, array, or
    primitive data type. Think JSON or YAML.

leaf
    An arbitrary description that is provided through VPD to the client.

source
    A namespace for descriptions, itself a description. Note that source
    descriptions should not be listed alongside leaf descriptions.

local name
    A namespace-relative description identifier. Any description can be
    identified by one or more source + local-name pairs.

canonical name
    A description identifier relative to the root namespace. Any description
    can be uniquely identified by a single canonical name.

URI
    A `Uniform Resource Identifier`__. May be reference or absolute.

__ https://www.w3.org/Addressing/

downstream
    From description host to client.

upstream
    From client to description host.


Serialization Format
--------------------

Any VPD implementation is required to support descriptions serialized in the
YAML_, JSON_, and compact_ formats. Other formats that
support key-value pairs and arrays can optionally be supported. For example,
MessagePack_.

.. _YAML: http://yaml.org/
.. _JSON: https://www.json.org/
.. _MessagePack: https://msgpack.org/

Compact Format
--------------

The VPD defines a compact binary representation for internal use and hash
computation. See each data object for details on its binary representation.
Note that some types have a different representation when serialized in "hash"
mode than when serialized for storage.

.. _compact: `Compact Format`_

Description Versioning
----------------------

The hash of a description is computed by first serializing the description
using the compact format in "hash" mode, as described above. The 32-bit
Castagnoli `cyclic redundancy check`_ checksum (CRC-32C) of the the serialized
description is then computed. This is the hash value. Additionally, the hash is
prefixed with the bytes ``0x6333`` as a version check. Future versions of the
VPD might define different hash types, identified by these first two bytes.

For example, the hash for the string_ "Hello, world!" (including length prefix)
is ``0x633304033195`` (or ``YzMEAzGV`` in base64).

.. _`cyclic redundancy check`: https://en.wikipedia.org/wiki/Cyclic_redundancy_check

Downstream Transmission
-----------------------

Clients can implement the downstream transmission of Upversion_ objects.
Sources can include information on how to connect within the ``down`` field.

Objects
=======

Bellow is a list of the different structures that can be described. Most are
tables of key-value mappings. These are represented as maps in both JSON and
YAML. The compact representation of one of these mappings is the concatenation
of each value's representation, in the order listed. Some objects have variants
which are distinguished in the compact form by prefixing a discriminant byte.
Objects that are not key-value mappings will specify their representation
below.

Source
------

======= ==================== ================================================
Key     Type                  What
======= ==================== ================================================
import  list_ (Import_)      Upstream sources
add     list_ (Addition_)    Leaf descriptions
ttl     maybe_ (real_)       The time-to-live of this description, in seconds
down    maybe_ (any_)        Data needed for downstream transmission
======= ==================== ================================================

Import
------

====== ===================== ======================================
Key    Type                  What
====== ===================== ======================================
path   URI_                  The location of the source description
name   Identifier_           The namespace identifier
check  maybe_ (Hash_)        The description's hash
type   maybe_ (MimeType_)    Description data format override
====== ===================== ======================================

The target of "path" is a Source_ object.

Addition
--------

One of the following.

====== ===================== ====================================
Key    Type                  What
====== ===================== ====================================
path   URI_                  The location of the description list
many   Identifier_           The class of associated descriptions
check  maybe_ (Hash_)        The list's hash
type   maybe_ (MimeType_)    Description data format override
====== ===================== ====================================

In this case, the referenced data is not a single description, but a key-value
list. Specifically, the target is represented as dict_ (Identifier_, any_). The
compact representation of this object should be prefixed by ``0x00`` as a
variant discriminant.

====== ===================== ====================================
Key    Type                  What
====== ===================== ====================================
path   URI_                  The location of the description
one    Identifier_           The class of the description
name   Name_                 The description's local name
check  maybe_ (Hash_)        The description's hash
====== ===================== ====================================

In this case, the target is represented as any_. The compact representation of
this object should be prefixed by ``0x01`` as a variant discriminant.

Upversion
---------

===== ========= =================================
Key   Type      What
===== ========= =================================
name  Name_     The local name of the description
check Hash_     The description's latest hash
===== ========= =================================

URI
---

A URI reference string.

This is represented as a string_.

Name
----

A list of identifiers, represented as a ":" delineated string_.

Identifier
----------

A non-empty unicode string. The following character classes are forbidden:

- Cc - Control
- Cf - Format
- Co - Private Use
- Cs - Surrogate
- Po - Other Punctuation
- Ps - Open Punctuation
- Pe - Close Punctuation
- Pi - Initial Punctuation
- Pf - Final Punctuation
- Zl - Line Separator
- Zp - Paragraph Separator
- Zs - Space Separator

This is represented as a string_.

Hash
----

A hash is a bytes_ object containing a hash__.

When represented in "hash" mode, this type behaves as the unit_ type rather
than the bytes_ type. This means that the hash of a description is independent
of hashes within that description, although it may be dependent on the
existence of a hash.

__ `Description Versioning`_

MimeType
--------

This is string_ object containing a `Multipurpose Internet Mail Extensions
(MIME) data type`__.

__ https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types

any
---

An any item can be one of:

============ =====================
discriminant value
============ =====================
``0x00``     ``null``
``0x08``     `false <bool_>`_
``0x09``     `true <bool_>`_
``0x10``     real_
``0x11``     integer_
``0x30``     string_
``0x50``     list_ (any_)
``0x70``     dict_ (string_, any_)
============ =====================

The representation is as follows.

+-----------+--------------------------------------------------------+
| JSON/YAML | Each value is represented as defined by its type.      |
|           | ``null`` is either represented as ``null`` in JSON and |
|           | as ``~`` in YAML, or left out of the parent collection |
|           | completely.                                            |
+-----------+--------------------------------------------------------+
| Compact   | The discriminant is a single byte. If the value is not |
|           | ``null``, ``true``, or ``false``, the discriminant is  |
|           | followed by the compact form of the value.             |
+-----------+--------------------------------------------------------+

string
------

bytes
-----



real
----

integer
-------

bool
----

maybe
-----

list
----

dict
----

default
-------

unit
----
