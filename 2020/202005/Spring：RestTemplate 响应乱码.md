# Spring：RestTemplate 响应乱码

## 环境

1. jdk 7
2. 4.3.24.RELEASE

## 原因

如果 RestTemplate 的响应是 String 类型，则会将响应交给 StringHttpMessageConverter 进行转化。
StringHttpMessageConverter 默认的编码是 ISO-8859-1，所以会出现乱码。

## 解决方案

```java
package jiangbo.springweb.rest;

import java.nio.charset.StandardCharsets;
import java.util.List;

import org.springframework.http.converter.HttpMessageConverter;
import org.springframework.http.converter.StringHttpMessageConverter;
import org.springframework.web.client.RestTemplate;

public class RestTemplateDemo03 {

    public static void main(String[] args) {

        String url = "http://www.baidu.com";

        String string = new RestTemplate().getForObject(url, String.class);
        System.out.println(string);

        RestTemplate template = new RestTemplate();
        List<HttpMessageConverter<?>> converters = template.getMessageConverters();

        for (int i = 0; i < converters.size(); i++) {
            if (converters.get(i) instanceof StringHttpMessageConverter) {
                converters.set(i, new StringHttpMessageConverter(StandardCharsets.UTF_8));
            }
        }

        String result = template.getForObject(url, String.class);
        System.out.println(result);
    }
}
```

## 附录

### StringHttpMessageConverter 源码

```java
/*
 * Copyright 2002-2017 the original author or authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.springframework.http.converter;

import java.io.IOException;
import java.io.UnsupportedEncodingException;
import java.nio.charset.Charset;
import java.util.ArrayList;
import java.util.List;

import org.springframework.http.HttpInputMessage;
import org.springframework.http.HttpOutputMessage;
import org.springframework.http.MediaType;
import org.springframework.util.StreamUtils;

/**
 * Implementation of {@link HttpMessageConverter} that can read and write strings.
 *
 * <p>By default, this converter supports all media types ({@code &#42;&#47;&#42;}),
 * and writes with a {@code Content-Type} of {@code text/plain}. This can be overridden
 * by setting the {@link #setSupportedMediaTypes supportedMediaTypes} property.
 *
 * @author Arjen Poutsma
 * @author Juergen Hoeller
 * @since 3.0
 */
public class StringHttpMessageConverter extends AbstractHttpMessageConverter<String> {

    // 在这里默认的编码是 ISO-8859-1
	public static final Charset DEFAULT_CHARSET = Charset.forName("ISO-8859-1");


	private volatile List<Charset> availableCharsets;

	private boolean writeAcceptCharset = true;


	/**
	 * A default constructor that uses {@code "ISO-8859-1"} as the default charset.
	 * @see #StringHttpMessageConverter(Charset)
	 */
	public StringHttpMessageConverter() {
        // 如果没有传递编码，则使用默认的编码
		this(DEFAULT_CHARSET);
	}

	/**
	 * A constructor accepting a default charset to use if the requested content
	 * type does not specify one.
	 */
	public StringHttpMessageConverter(Charset defaultCharset) {
		super(defaultCharset, MediaType.TEXT_PLAIN, MediaType.ALL);
	}


	/**
	 * Indicates whether the {@code Accept-Charset} should be written to any outgoing request.
	 * <p>Default is {@code true}.
	 */
	public void setWriteAcceptCharset(boolean writeAcceptCharset) {
		this.writeAcceptCharset = writeAcceptCharset;
	}


	@Override
	public boolean supports(Class<?> clazz) {
		return String.class == clazz;
	}

	@Override
	protected String readInternal(Class<? extends String> clazz, HttpInputMessage inputMessage) throws IOException {
		Charset charset = getContentTypeCharset(inputMessage.getHeaders().getContentType());
		return StreamUtils.copyToString(inputMessage.getBody(), charset);
	}

	@Override
	protected Long getContentLength(String str, MediaType contentType) {
		Charset charset = getContentTypeCharset(contentType);
		try {
			return (long) str.getBytes(charset.name()).length;
		}
		catch (UnsupportedEncodingException ex) {
			// should not occur
			throw new IllegalStateException(ex);
		}
	}

	@Override
	protected void writeInternal(String str, HttpOutputMessage outputMessage) throws IOException {
		if (this.writeAcceptCharset) {
			outputMessage.getHeaders().setAcceptCharset(getAcceptedCharsets());
		}
		Charset charset = getContentTypeCharset(outputMessage.getHeaders().getContentType());
		StreamUtils.copy(str, charset, outputMessage.getBody());
	}


	/**
	 * Return the list of supported {@link Charset}s.
	 * <p>By default, returns {@link Charset#availableCharsets()}.
	 * Can be overridden in subclasses.
	 * @return the list of accepted charsets
	 */
	protected List<Charset> getAcceptedCharsets() {
		if (this.availableCharsets == null) {
			this.availableCharsets = new ArrayList<Charset>(
					Charset.availableCharsets().values());
		}
		return this.availableCharsets;
	}

	private Charset getContentTypeCharset(MediaType contentType) {
		if (contentType != null && contentType.getCharset() != null) {
			return contentType.getCharset();
		}
		else {
			return getDefaultCharset();
		}
	}
}
```