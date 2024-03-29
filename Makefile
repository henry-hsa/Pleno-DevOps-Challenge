# Write a settings for Makefile for service 1 and 2
# hide the project id for security reason

## for creating cluster
create-cluster:
	gcloud beta container \
	    --project "hsa-dev-xxxxx" \
		clusters create-auto "gke-cluster" \ 
		--regions "asia-southeast2" \
		--release-channel "regular" \
		--network "projects/hsa-dev-xxxxx/global/networks/default" \
		--subnetwork "projects/hsa-dev-xxxxx/regions/asia-southeast2/subnetworks/default" \
		--clusteripv4-cidr "10.28.0.0/16" \
		--binauthz-evaluation-mode=DISABLED 

# deploment and services for service1 and service2 
# alternatively for kube deployment excute directy from CD pipeline
deploy-k8s:
    kubectl apply -f kube/k8.yaml

