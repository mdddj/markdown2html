# markdown2html

markdown to html dart lib

## Getting Started

```yaml
markdown2html: any
```

## api

#### init

```dart
void main(){
  markdownInit();
}
```


### 1. markdown to html



```dart

import 'package:markdown2html/api/md.dart';

String html = await markdow2Html(str: "## hello world");
```

output 
```dart
<h2>hello</h2>
```

### 2. markdown to html for options


```dart
markdown2HtmlWithOptions(
  str: "## hello world",
  options:MkCompileOptions(),
  parseOptions:MkParseOptions(),
);
```

# example

```dart
import 'package:flutter/material.dart';
import 'package:markdown2html/api/md.dart';
import 'package:markdown2html/frb_generated.dart';
import 'dart:async';

import 'package:markdown2html/markdown2html.dart' as markdown2html;
import 'package:markdown2html/markdown2html.dart';

final _testmk1 = """
## hello world
""";

void main() {
  markdownInit();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  final controller = TextEditingController(text: _testmk1);

  String html = "";
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
          appBar: AppBar(
            title: const Text('Native Packages'),
          ),
          body: SingleChildScrollView(
            padding: const EdgeInsets.all(12),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.center,
              children: [
                TextField(
                  controller: controller,
                  maxLines: 10,
                  minLines: 5,
                  decoration: const InputDecoration(
                      helperText: "input you markdown text "),
                ),
                FilledButton(
                    onPressed: () async {
                      html = await markdow2Html(str: controller.text);
                      setState(() {});
                      debugPrint("$html");
                      markdown2HtmlWithOptions(str: controller.text);
                    },
                    child: const Text("markdown to html")),
                const Text('output html :'),
                Text(html)
              ],
            ),
          )),
    );
  }
}

```


