FROM python:3.6
MAINTAINER Sumner Evans

# Copy all of the current working directory to /visplay.
WORKDIR /visplay
ADD . /visplay

# Install Visplay
RUN pip install .

# Install dependencies
RUN apt-get update && apt-get upgrade && apt-get install -y libmpv-dev

# Expose port 80 and run the application.
EXPOSE 80
CMD ["visplay"]
