# CS4414 HW 2 - Fast edit distances

You should complete the following questions in your final submission:

## Step 0: Build modes

1. For the dictionary `examples/popular.txt`, how long does it take to
   run the code in the `debug` compilation mode?  What about the
   `release` mode?
   for the regular mode it was 110.798 seconds
   for the released mode it was only 3.81

2. Based on the relative sizes of the dictionaries, estimate how long
   you think it would take to run in the two modes for the
   `examples/enable1.txt` dictionary.  Test your hypothesis in release
   mode; how long does it actually take to run?
   enable1.txt debug estimate:   756.75 s 
   enable1.txt release estimate: 26.02 s

## Step 1: Blocking

1. What is the estimated memory footprint for the two dictionaries
   (`popular.txt` and `enable1.txt`)?  Include the storage for the
   `String` metadata in your accounting (don't worry about storage for
   the allocator's data structures).
   I think for the enable1 it should take 4.15 MB while for the popular it should be around 0.61MB.. The String metadata should contain 24 bytes of data.

2. Complete the code for the blocked variant.  You should see a speed
   difference that is noticeable, but not enormous.  What difference
   do you see?
   Naive version is 3.8 seconds while the blocked version is 3.4 seconds.

## Step 2: Removing indirection

1. What is the speed difference compared to the method in step 2?
   My packed version ran in 1.6 seconds so its about 1.86 seconds faster.

2. The longest word in `enable1.txt` is 28 characters, but most are
   shorter.  If you write your code to reserve one byte for the word
   length at the beginning, what type of performance improvement do
   you see?
   It took 291 seconds rather than the 74 seconds before. The preformance was slower.

## Step 3: Packed representation

What do you see?  Is your version any faster than the
method you explored in Step 2?
This is much faster than any of the optimazation before this run only took 0.87 seconds.
## Step 4: Speed demon

Describe the steps that you took to get to your final optimized
version!
