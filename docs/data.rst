Visplay Data Protocol
=====================

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
YAML_ or JSON_ formats. Other formats that support key-value pairs and arrays
can optionally be supported. For example, MessagePack_.

.. _YAML: http://yaml.org/
.. _JSON: https://www.json.org/
.. _MessagePack: https://msgpack.org/

Downstream Transmission
-----------------------

.. TODO

VDP Objects
===========

Source
------

======= ================= ================================================
Key     Type              What
======= ================= ================================================
import  list(`Import`_)   Upstream sources
add     list(`Addition`_) Leaf descriptions
ttl     maybe(number)     The time-to-live of this description, in seconds
======= ================= ================================================

Import
------

====== ================== ======================================
Key    Type               What
====== ================== ======================================
path   `URI`_             The location of the source description
name   `Identifier`_      The namespace identifier
check  maybe(`Checksum`_) The description's checksum
type   maybe(`MimeType`_) Description data format override
====== ================== ======================================

Addition
--------

One of the following.

====== ================== ====================================
Key    Type               What
====== ================== ====================================
path   `URI`_             The location of the description list
many   `Identifier`_      The class of associated descriptions
check  maybe(`Checksum`_) The list's checksum
type   maybe(`MimeType`_) Description data format override
====== ================== ====================================

In this case, the referenced data is not a single description, but a key-value
list. The keys are of the type `Identifier`_, and the values are leaf objects.

====== ================== ====================================
Key    Type               What
====== ================== ====================================
path   `URI`_             The location of the description
one    `Identifier`_      The class of the description
name   `Name`_            The description's local name
check  maybe(`Checksum`_) The description's checksum
====== ================== ====================================

In this case, the referenced data is a leaf object.

URI
---

A URI reference string.

Name
----

A colon-deliniated unicode string of identifiers.

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

Checksum
--------

A base64-encoded string.

.. TODO

MimeType
--------

A `Multipurpose Internet Mail Extensions (MIME) type`__.

__ https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types
