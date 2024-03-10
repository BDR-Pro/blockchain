import tensorflow as tf

from tensorflow.keras.layers import Dense, Flatten

from tensorflow.keras.models import Sequential

from tensorflow.keras.datasets import mnist

from tensorflow.keras.utils import to_categorical

import sys

print(sys.executable)

print(sys.version)

print("Tensorflow version: ", tf.__version__)

from tensorflow.python.profiler.model_analyzer import profile

from tensorflow.python.profiler.option_builder import ProfileOptionBuilder



def estimate_model_teraflops(model):

  forward_pass = tf.function(model.call, input_signature=[tf.TensorSpec(shape=(1,) + model.input_shape[1:])])

  graph_info = profile(forward_pass.get_concrete_function().graph, options=ProfileOptionBuilder.float_operation())

  flops = graph_info.total_float_ops

  return flops  / 10**12

# Load the MNIST data

(x_train, y_train), (x_test, y_test) = mnist.load_data()



# Normalize the data

x_train, x_test = x_train / 255.0, x_test / 255.0



# Convert class vectors to binary class matrices (one-hot encoding)

y_train = to_categorical(y_train, 10)

y_test = to_categorical(y_test, 10)

# Build the model

model = Sequential([

  Flatten(input_shape=(28, 28)),

  Dense(128, activation='relu'),

  Dense(10, activation='softmax')

])



# Compile the model

model.compile(optimizer='adam',

              loss='categorical_crossentropy',

              metrics=['accuracy'])

# Train the model

model.fit(x_train, y_train, epochs=5)



# Evaluate the model

loss, accuracy = model.evaluate(x_test, y_test)

print(f'Loss: {loss}, Accuracy: {accuracy}')



# Estimate the TeraFLOPs (this part is highly theoretical and simplified)

teraFLOP = estimate_model_teraflops(model)

print(f"teraFLOP: {teraFLOP}")
