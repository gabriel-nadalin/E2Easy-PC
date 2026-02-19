#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "e2easy.h"
#include <cjson/cJSON.h>

// Helper function to write string to file
int write_to_file(const char* filename, const char* content) {
    FILE* file = fopen(filename, "w");
    if (!file) {
        fprintf(stderr, "Failed to open file: %s\n", filename);
        return 0;
    }
    fprintf(file, "%s", content);
    fclose(file);
    return 1;
}

int main() {
    printf("Testing E2Easy FFI functions...\n");
    
    // Test 1: Create E2Easy instance
    printf("\n1. Creating E2Easy instance...\n");
    E2Easy_t* instance = e2easy_new();
    if (instance == NULL) {
        printf("   ERROR: Failed to create E2Easy instance\n");
        return 1;
    }
    printf("   SUCCESS: E2Easy instance created\n");
    
    // Test 2: Vote
    printf("\n2. Testing vote function...\n");
    const char* votes_json = "[{\"contest\":0,\"choice\":1},{\"contest\":1,\"choice\":2}]";
    
    JsonResult_t vote_result = e2easy_vote(&instance, votes_json);
    if (!vote_result.success) {
        printf("   ERROR: Vote function failed: %s\n", vote_result.data);
        json_result_free(vote_result);
        e2easy_free(instance);
        return 1;
    }
    printf("   SUCCESS: Vote completed\n");
    printf("   Result: %s\n", vote_result.data);
    json_result_free(vote_result);
    
    // Test 3: Challenge
    printf("\n3. Testing challenge function...\n");
    JsonResult_t challenge_result = e2easy_challenge(&instance);
    if (!challenge_result.success) {
        printf("   ERROR: Challenge function failed: %s\n", challenge_result.data);
        json_result_free(challenge_result);
        e2easy_free(instance);
        return 1;
    }
    printf("   SUCCESS: Challenge completed\n");
    printf("   Result: %s\n", challenge_result.data);
    json_result_free(challenge_result);
    
    // Test 4: Vote again (for cast)
    printf("\n4. Voting again for cast test...\n");
    JsonResult_t vote_result2 = e2easy_vote(&instance, votes_json);
    if (!vote_result2.success) {
        printf("   ERROR: Second vote failed: %s\n", vote_result2.data);
        json_result_free(vote_result2);
        e2easy_free(instance);
        return 1;
    }
    printf("   SUCCESS: Second vote completed\n");
    printf("   Result: %s\n", vote_result2.data);
    json_result_free(vote_result2);
    
    // Test 5: Cast
    printf("\n5. Testing cast function...\n");
    JsonResult_t cast_result = e2easy_cast(&instance);
    if (!cast_result.success) {
        printf("   ERROR: Cast function failed: %s\n", cast_result.data);
        json_result_free(cast_result);
        e2easy_free(instance);
        return 1;
    }
    printf("   SUCCESS: Cast completed\n");
    printf("   Result: %s\n", cast_result.data);
    json_result_free(cast_result);
    
    // Test 6: Tally and write individual files
    printf("\n6. Testing tally function...\n");
    JsonResult_t tally_result = e2easy_tally(&instance);
    if (!tally_result.success) {
        printf("   ERROR: Tally function failed: %s\n", tally_result.data);
        json_result_free(tally_result);
        e2easy_free(instance);
        return 1;
    }
    printf("   SUCCESS: Tally completed\n");
    
    // Parse the JSON result
    cJSON* tally_json = cJSON_Parse(tally_result.data);
    if (tally_json == NULL) {
        printf("   ERROR: Failed to parse tally JSON\n");
        json_result_free(tally_result);
        e2easy_free(instance);
        return 1;
    }
    
    // Extract individual components and write to files
    printf("\n7. Writing tally results to individual files...\n");
    
    cJSON* rdv_prime = cJSON_GetObjectItem(tally_json, "rdv_prime");
    if (rdv_prime) {
        char* rdv_prime_str = cJSON_Print(rdv_prime);
        if (write_to_file("./outputs/rdv_prime.json", rdv_prime_str)) {
            printf("   SUCCESS: rdv_prime.json written\n");
        }
        free(rdv_prime_str);
    }
    
    cJSON* rdcv = cJSON_GetObjectItem(tally_json, "rdcv");
    if (rdcv) {
        char* rdcv_str = cJSON_Print(rdcv);
        if (write_to_file("./outputs/rdcv.json", rdcv_str)) {
            printf("   SUCCESS: rdcv.json written\n");
        }
        free(rdcv_str);
    }
    
    cJSON* rdcv_prime = cJSON_GetObjectItem(tally_json, "rdcv_prime");
    if (rdcv_prime) {
        char* rdcv_prime_str = cJSON_Print(rdcv_prime);
        if (write_to_file("./outputs/rdcv_prime.json", rdcv_prime_str)) {
            printf("   SUCCESS: rdcv_prime.json written\n");
        }
        free(rdcv_prime_str);
    }
    
    cJSON* zkp = cJSON_GetObjectItem(tally_json, "zkp");
    if (zkp) {
        char* zkp_str = cJSON_Print(zkp);
        if (write_to_file("./outputs/zkp_output.json", zkp_str)) {
            printf("   SUCCESS: zkp_output.json written\n");
        }
        free(zkp_str);
    }
    
    // Cleanup
    cJSON_Delete(tally_json);
    json_result_free(tally_result);
    
    // Test 7: Free instance
    printf("\n8. Freeing E2Easy instance...\n");
    e2easy_free(instance);
    printf("   SUCCESS: Instance freed\n");
    
    printf("\nAll tests passed! Files written to ./outputs/\n");
    return 0;
}